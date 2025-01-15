use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path;
use std::result::Result;

pub use crate::shapes::*;
pub use crate::units::*;

const N_OBJ_RESERVED: usize = 2; // First two objects are reserved for pages.

#[derive(Debug)]
pub struct Generator {
    file_path: path::PathBuf,
    pdf: Vec<u8>,           // PDF binary content
    pdf_pre: Vec<u8>,       // PDF binary content before the first page
    offsets: Vec<usize>,    // Object offsets for xref
    pre_offset: usize,      // Offset before the first page
    content_stream: String, // Content stream to accumulate drawing commands
    pages: Vec<usize>,      // Page object numbers
}

impl Generator {
    pub fn new(file_path: path::PathBuf) -> Self {
        Self {
            file_path,
            pdf: Vec::new(),
            pdf_pre: Vec::new(),
            offsets: vec![0; N_OBJ_RESERVED], // First two objects are reserved for pages.
            pre_offset: 0,
            content_stream: String::new(),
            pages: Vec::new(),
        }
    }

    pub fn write_pdf(&mut self) -> Result<(), Box<dyn Error>> {
        self.initialize_pdf();
        self.finalize_pdf();
        let mut file = File::create(&self.file_path)?;
        file.write_all(&self.pdf_pre)?;
        file.write_all(&self.pdf)?;
        Ok(())
    }

    fn initialize_pdf(&mut self) {
        // add remaining content
        self.add_content();

        self.pdf_pre.extend(b"%PDF-1.5\n");

        // Catalog object
        self.add_pre_object("<< /Type /Catalog /Pages 2 0 R >>", 0);

        // Pages object
        let pages_kids: String = self
            .pages
            .iter()
            .map(|page| format!("{} 0 R ", page))
            .collect::<String>()
            .trim()
            .to_string();
        self.add_pre_object(
            &format!(
                "<< /Type /Pages /Kids [{}] /Count {} >>",
                pages_kids,
                self.pages.len()
            ),
            1,
        );
    }

    fn finalize_pdf(&mut self) {
        // Xref table
        let xref_start = self.pdf_pre.len() + self.pdf.len();
        self.pdf.extend(b"xref\n");
        self.pdf
            .extend(format!("0 {}\n0000000000 65535 f \n", self.offsets.len() + 1).as_bytes());
        for (i, offset) in self.offsets.iter().enumerate() {
            self.pdf.extend(
                format!(
                    "{:010} 00000 {} \n",
                    offset
                        + if i >= N_OBJ_RESERVED && (*offset > 0 || i == N_OBJ_RESERVED) {
                            self.pre_offset
                        } else {
                            0
                        },
                    if *offset == 0 && i > N_OBJ_RESERVED {
                        'f'
                    } else {
                        'n'
                    }
                )
                .as_bytes(),
            );
        }

        // Trailer
        self.pdf.extend(b"trailer\n");
        self.pdf.extend(
            format!(
                "<< /Root 1 0 R /Size {} >>\nstartxref\n{}\n%%EOF\n",
                self.offsets.len() + 1,
                xref_start
            )
            .as_bytes(),
        );
    }

    fn add_object(&mut self, content: &str) {
        self.offsets.push(self.pdf.len()); // Track offset
        self.pdf
            .extend(format!("{} 0 obj\n{}\nendobj\n", self.offsets.len(), content).as_bytes());
    }

    fn add_pre_object(&mut self, content: &str, n_obj: usize) {
        self.offsets[n_obj] = self.pdf_pre.len(); // Track offset
        self.pdf_pre
            .extend(format!("{} 0 obj\n{}\nendobj\n", n_obj + 1, content).as_bytes());
        self.pre_offset = self.pdf_pre.len();
    }

    pub fn add_page_a4(&mut self) {
        self.add_page_with_size(Mm(210.0), Mm(297.0));
    }

    pub fn add_page_letter(&mut self) {
        self.add_page_with_size(Inch(8.5), Inch(11.0));
    }

    pub fn add_page_with_size<L: Length>(&mut self, width: L, height: L) {
        if self.pages.len() > 0 {
            self.add_content(); // Add content for the previous page
        }

        self.pages.push(self.offsets.len() + 1);

        // Page object
        self.add_object(&format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents {} 0 R >>",
            width.to_points(),
            height.to_points(),
            self.offsets.len() + 2
        ));
    }

    fn add_content(&mut self) {
        self.add_object(&format!(
            "<< /Length {} >>\nstream\n{}\nendstream",
            self.content_stream.len(),
            self.content_stream
        ));
        self.content_stream.clear();
    }

    pub fn add_rectangle<L: Length, A: Angle>(
        &mut self,
        x: L,
        y: L,
        width: L,
        height: L,
        angle: A,
    ) {
        let cos_theta = angle.to_radians().cos();
        let sin_theta = angle.to_radians().sin();
        let cx = x.to_points() + width.to_points() / 2.0;
        let cy = y.to_points() + height.to_points() / 2.0;
        let translate_x = cx - cos_theta * cx + sin_theta * cy;
        let translate_y = cy - sin_theta * cx - cos_theta * cy;
        self.content_stream.push_str(&format!(
            "{} {} {} {} {} {} cm\n{} {} {} {} re f\n",
            cos_theta,
            sin_theta,
            -sin_theta,
            cos_theta,
            translate_x,
            translate_y,
            x.to_points(),
            y.to_points(),
            width.to_points(),
            height.to_points()
        ));
    }

    pub fn add_circle<L: Length>(&mut self, cx: L, cy: L, radius: L) {
        // self.content_stream.push_str(&format!(
        //     "{} w\n1 J\n{} {} m\n{} {} l\nS\n",
        //     radius.to_points() * 2.0,
        //     cx.to_points(),
        //     cy.to_points(),
        //     cx.to_points(),
        //     cy.to_points()
        // ));
        self.add_line(cx, cy, cx, cy, radius * 2.0, CapType::Round);
    }

    pub fn add_line<L: Length>(&mut self, x1: L, y1: L, x2: L, y2: L, width: L, cap_type: CapType) {
        self.content_stream.push_str(&format!(
            "{} w\n{} J\n{} {} m\n{} {} l\nS\n",
            width.to_points(),
            cap_type.to_int(),
            x1.to_points(),
            y1.to_points(),
            x2.to_points(),
            y2.to_points()
        ));
    }

    pub fn line<L: Length>(&mut self, x1: L, y1: L, x2: L, y2: L) -> Shape {
        Shape {
            content_stream: Some(&mut self.content_stream),
            enum_type: ShapeType::Line,
            x1: Some(x1.to_points()),
            y1: Some(y1.to_points()),
            x2: Some(x2.to_points()),
            y2: Some(y2.to_points()),
            ..Default::default()
        }
    }

    pub fn add_polygon<L: Length>(&mut self, points: &[(L, L)]) {
        if points.is_empty() {
            return;
        }
        let mut iter = points.iter();
        if let Some((x, y)) = iter.next() {
            self.content_stream
                .push_str(&format!("{} {} m\n", x.to_points(), y.to_points()));
        }
        for (x, y) in iter {
            self.content_stream
                .push_str(&format!("{} {} l\n", x.to_points(), y.to_points()));
        }
        self.content_stream.push_str("h f\n"); // Close the path and fill
    }
}
