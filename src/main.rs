use shapdf::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut generator = Generator::new("output/shapes.pdf".into());
    generator.add_page_letter();
    // generator.add_circle(Mm(20.), Mm(20.), Mm(10.));
    generator.add_line(
        Pt(100.),
        Pt(100.),
        Pt(200.),
        Pt(200.),
        Pt(1.5),
        LineCap::Butt,
    );
    generator.add_page_letter();
    generator.add_rectangle(Mm(50.), Mm(180.), Mm(50.), Mm(30.), Degree(30.));
    generator.add_page_a4();
    generator.write_pdf()?;
    println!("PDF generated successfully!");
    Ok(())
}
