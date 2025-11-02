#[cfg(feature = "compress")]
use flate2::{write::ZlibEncoder, Compression};
use once_cell::sync::Lazy;
use std::error::Error;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(target_arch = "wasm32")]
use std::io;
use std::io::Write;
use std::path;
use std::sync::Mutex;

pub use crate::shapes::*;
pub use crate::units::*;

const N_OBJ_RESERVED: usize = 2; // First two objects are reserved for pages.

static DEFAULT_PAGE_WIDTH: Lazy<Mutex<f64>> = Lazy::new(|| Inch(8.5).to_points().into());
static DEFAULT_PAGE_HEIGHT: Lazy<Mutex<f64>> = Lazy::new(|| Inch(11.0).to_points().into());

/// PDF generator.
#[derive(Debug)]
pub struct Generator {
    #[cfg(not(target_arch = "wasm32"))]
    file_path: path::PathBuf,
    #[cfg(target_arch = "wasm32")]
    _file_path: path::PathBuf,
    pdf: Vec<u8>,            // PDF binary content
    pdf_pre: Vec<u8>,        // PDF binary content before the first page
    offsets: Vec<usize>,     // Object offsets for xref
    pre_offset: usize,       // Offset before the first page
    content_stream: Vec<u8>, // Content stream to accumulate drawing commands
    pages: Vec<usize>,       // Page object numbers
    finished: bool,          // Whether the PDF was finalized
}

impl Generator {
    pub fn new(file_path: path::PathBuf) -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            file_path,
            #[cfg(target_arch = "wasm32")]
            _file_path: file_path,
            pdf: Vec::new(),
            pdf_pre: Vec::new(),
            offsets: vec![0; N_OBJ_RESERVED], // First two objects are reserved for pages.
            pre_offset: 0,
            content_stream: Vec::new(),
            pages: Vec::new(),
            finished: false,
        }
    }

    fn ensure_finalized(&mut self) {
        if self.finished {
            return;
        }
        self.initialize_pdf();
        self.finalize_pdf();
        self.finished = true;
    }

    fn collect_pdf_bytes(&mut self) -> Vec<u8> {
        self.ensure_finalized();
        let mut bytes = Vec::with_capacity(self.pdf_pre.len() + self.pdf.len());
        bytes.extend_from_slice(&self.pdf_pre);
        bytes.extend_from_slice(&self.pdf);
        bytes
    }

    pub fn to_pdf_bytes(&mut self) -> Vec<u8> {
        self.collect_pdf_bytes()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn write_pdf(&mut self) -> Result<(), Box<dyn Error>> {
        let bytes = self.collect_pdf_bytes();
        // create directories if not exist
        if let Some(dir) = self.file_path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut file = File::create(&self.file_path)?;
        file.write_all(&bytes)?;
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn write_pdf(&mut self) -> Result<(), Box<dyn Error>> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "write_pdf is not supported on wasm targets",
        )
        .into())
    }

    fn initialize_pdf(&mut self) {
        // add remaining content
        self.add_content();

        self.pdf_pre.extend(b"%PDF-1.5\n");

        // Catalog object
        self.add_pre_object(b"<< /Type /Catalog /Pages 2 0 R >>", 0);

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
            )
            .as_bytes(),
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

    fn add_object(&mut self, content: &[u8]) {
        self.offsets.push(self.pdf.len()); // Track offset
        self.pdf
            .extend_from_slice(format!("{} 0 obj\n", self.offsets.len()).as_bytes());
        self.pdf.extend_from_slice(content);
        self.pdf.extend_from_slice(b"\nendobj\n");
    }

    fn add_pre_object(&mut self, content: &[u8], n_obj: usize) {
        self.offsets[n_obj] = self.pdf_pre.len(); // Track offset
        self.pdf_pre
            .extend_from_slice(format!("{} 0 obj\n", n_obj + 1).as_bytes());
        self.pdf_pre.extend_from_slice(content);
        self.pdf_pre.extend_from_slice(b"\nendobj\n");
        self.pre_offset = self.pdf_pre.len();
    }

    pub fn add_page(&mut self) {
        self.add_page_with_size(
            Pt(*DEFAULT_PAGE_WIDTH.lock().unwrap()),
            Pt(*DEFAULT_PAGE_HEIGHT.lock().unwrap()),
        );
    }

    pub fn add_page_a4(&mut self) {
        self.add_page_with_size(Mm(210.0), Mm(297.0));
    }

    pub fn add_page_a4_landscape(&mut self) {
        self.add_page_with_size(Mm(297.0), Mm(210.0));
    }

    pub fn add_page_letter(&mut self) {
        self.add_page_with_size(Inch(8.5), Inch(11.0));
    }

    pub fn add_page_letter_landscape(&mut self) {
        self.add_page_with_size(Inch(11.0), Inch(8.5));
    }

    pub fn add_page_with_size<L: Length>(&mut self, width: L, height: L) {
        if self.pages.len() > 0 {
            self.add_content(); // Add content for the previous page
        }

        self.pages.push(self.offsets.len() + 1);

        // Page object
        self.add_object(
            &format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents {} 0 R >>",
                width.to_points(),
                height.to_points(),
                self.offsets.len() + 2
            )
            .as_bytes(),
        );
    }

    /// Compress the content stream using the ZlibEncoder.
    #[cfg(feature = "compress")]
    fn compress_stream(stream: &mut Vec<u8>) -> bool {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(stream).unwrap();
        let compressed = encoder.finish().unwrap();
        if compressed.len() + 21 < stream.len() {
            // 21 is for the length of "/Filter /FlateDecode"
            stream.clear();
            stream.extend_from_slice(&compressed);
            return true;
        } else {
            return false;
        }
    }

    fn add_content(&mut self) {
        #[cfg(feature = "compress")]
        let flate_decode = Self::compress_stream(&mut self.content_stream);
        #[cfg(not(feature = "compress"))]
        let flate_decode = false;
        let len = self.content_stream.len();
        let mut content: Vec<u8> = format!(
            "<< /Length {} {}>>\nstream\n",
            len,
            if flate_decode {
                "/Filter /FlateDecode "
            } else {
                ""
            }
        )
        .as_bytes()
        .to_vec();
        content.extend_from_slice(&self.content_stream);
        content.extend_from_slice(b"\nendstream\n");
        self.add_object(&content);
        self.content_stream.clear();
    }

    pub fn line(
        &mut self,
        x1: impl Length,
        y1: impl Length,
        x2: impl Length,
        y2: impl Length,
    ) -> Shape<'_> {
        Shape {
            content_stream: Some(&mut self.content_stream),
            enum_type: ShapeType::Line,
            x: vec![x1.to_points(), x2.to_points()],
            y: vec![y1.to_points(), y2.to_points()],
            ..Default::default()
        }
    }

    pub fn circle(&mut self, x: impl Length, y: impl Length, radius: impl Length) -> Shape<'_> {
        Shape {
            content_stream: Some(&mut self.content_stream),
            enum_type: ShapeType::Circle,
            x: vec![x.to_points()],
            y: vec![y.to_points()],
            radius: Some(radius.to_points()),
            ..Default::default()
        }
    }

    pub fn rectangle(
        &mut self,
        x: impl Length,
        y: impl Length,
        width: impl Length,
        height: impl Length,
    ) -> Shape<'_> {
        Shape {
            content_stream: Some(&mut self.content_stream),
            enum_type: ShapeType::Rectangle,
            x: vec![x.to_points(), width.to_points()],
            y: vec![y.to_points(), height.to_points()],
            ..Default::default()
        }
    }

    // pub fn add_polygon<L: Length>(&mut self, points: &[(L, L)]) {
    //     if points.is_empty() {
    //         return;
    //     }
    //     let mut iter = points.iter();
    //     if let Some((x, y)) = iter.next() {
    //         self.content_stream
    //             .push_str(&format!("{} {} m\n", x.to_points(), y.to_points()));
    //     }
    //     for (x, y) in iter {
    //         self.content_stream
    //             .push_str(&format!("{} {} l\n", x.to_points(), y.to_points()));
    //     }
    //     self.content_stream.push_str("h f\n"); // Close the path and fill
    // }

    pub fn get_default_page_size() -> (Pt, Pt) {
        (
            Pt(*DEFAULT_PAGE_WIDTH.lock().unwrap()),
            Pt(*DEFAULT_PAGE_HEIGHT.lock().unwrap()),
        )
    }

    pub fn set_default_page_size(width: impl Length, height: impl Length) {
        *DEFAULT_PAGE_WIDTH.lock().unwrap() = width.to_points();
        *DEFAULT_PAGE_HEIGHT.lock().unwrap() = height.to_points();
    }

    pub fn get_default_width() -> Pt {
        Shape::get_default_width()
    }

    pub fn set_default_width(width: impl Length) {
        Shape::set_default_width(width);
    }

    pub fn set_default_cap_type(cap_type: CapType) {
        Shape::set_default_cap_type(cap_type);
    }

    pub fn set_default_color(color: impl Color) {
        Shape::set_default_color(color);
    }

    pub fn set_default_angle(angle: impl Angle) {
        Shape::set_default_angle(angle);
    }
}
