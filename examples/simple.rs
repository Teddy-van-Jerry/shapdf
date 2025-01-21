use shapdf::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut generator = Generator::new("output/shapes.pdf".into());
    // Generator::set_default_page_size(Pt(800.), Pt(800.));
    generator.add_page();
    generator
        .circle(Mm(20.), Mm(20.), Mm(10.))
        .with_color(NamedColor("blue"))
        .draw();
    generator
        .line(Pt(500.), Pt(600.), Pt(300.), Pt(400.))
        .with_width(Mm(10.))
        .with_cap_type(CapType::Round)
        .with_color(NamedColor("red"))
        .draw();

    generator.add_page_letter();
    generator
        .rectangle(Mm(80.), Mm(180.), Mm(50.), Mm(30.))
        .with_anchor(Anchor::Center)
        .with_angle(Degree(30.))
        .draw();
    generator
        .circle(Mm(80.), Mm(180.), Mm(1.))
        .with_color(NamedColor("green"))
        .draw();
    generator.add_page_a4();
    generator.write_pdf()?;
    println!("PDF generated successfully!");
    Ok(())
}
