use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path;
use std::result::Result;

pub use crate::units::*;

#[derive(Debug)]
pub struct Generator {
    file_path: path::PathBuf,
    pdf: Vec<u8>,           // PDF binary content
    offsets: Vec<usize>,    // Object offsets for xref
    content_stream: String, // Content stream to accumulate drawing commands
}

#[derive(Debug)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl LineCap {
    fn to_int(&self) -> i32 {
        match self {
            LineCap::Butt => 0,
            LineCap::Round => 1,
            LineCap::Square => 2,
        }
    }
}

impl Generator {
    pub fn new(file_path: path::PathBuf) -> Self {
        Self {
            file_path,
            pdf: Vec::new(),
            offsets: Vec::new(),
            content_stream: String::new(),
        }
    }

    pub fn write_pdf(&mut self) -> Result<(), Box<dyn Error>> {
        self.initialize_pdf();
        self.add_content();
        self.finalize_pdf();
        let mut file = File::create(&self.file_path)?;
        file.write_all(&self.pdf)?;
        Ok(())
    }

    fn initialize_pdf(&mut self) {
        self.pdf.extend(b"%PDF-1.5\n");
    }

    fn finalize_pdf(&mut self) {
        // Xref table
        let xref_start = self.pdf.len();
        self.pdf.extend(b"xref\n");
        self.pdf
            .extend(format!("0 {}\n0000000000 65535 f \n", self.offsets.len() + 1).as_bytes());
        for offset in &self.offsets {
            self.pdf
                .extend(format!("{:010} 00000 n \n", offset).as_bytes());
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

    pub fn add_page_a4(&mut self) {
        self.add_page_with_size(Mm(210.0), Mm(297.0));
    }

    pub fn add_page_letter(&mut self) {
        self.add_page_with_size(Inch(8.5), Inch(11.0));
    }

    pub fn add_page_with_size<L: Length>(&mut self, width: L, height: L) {
        // Catalog object
        self.add_object("<< /Type /Catalog /Pages 2 0 R >>");

        // Pages object
        self.add_object(&format!("<< /Type /Pages /Kids [3 0 R] /Count 1 >>"));

        // Page object
        self.add_object(&format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents 4 0 R >>",
            width.to_points(),
            height.to_points()
        ));
    }

    fn add_content(&mut self) {
        self.add_object(&format!(
            "<< /Length {} >>\nstream\n{}\nendstream",
            self.content_stream.len(),
            self.content_stream
        ));
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
        self.content_stream.push_str(&format!(
            "{} w\n1 J\n{} {} m\n{} {} l\nS\n",
            radius.to_points() * 2.0,
            cx.to_points(),
            cy.to_points(),
            cx.to_points(),
            cy.to_points()
        ));
    }

    pub fn add_line<L: Length>(&mut self, x1: L, y1: L, x2: L, y2: L, width: L, cap_type: LineCap) {
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
