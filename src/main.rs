use shapdf::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut generator = Generator::new("output/shapes.pdf".into());
    generator.add_page_letter();
    generator.add_circle(Mm(20.), Mm(20.), Mm(10.));
    generator
        .line(
            Pt(500.),
            Pt(600.),
            Pt(300.),
            Pt(400.),
        )
        .with_width(Mm(10.))
        .with_cap_type(CapType::Round)
        .with_color((1., 0., 0.))
        .draw();

    generator.add_page_letter();
    generator.add_rectangle(Mm(50.), Mm(180.), Mm(50.), Mm(30.), Degree(30.));
    generator.add_page_a4();
    generator.write_pdf()?;
    println!("PDF generated successfully!");
    Ok(())
}
