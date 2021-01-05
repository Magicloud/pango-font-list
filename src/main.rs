use pangocairo::FontMap;
use pango::FontMapExt;
use pango::FontFamilyExt;

fn main() {
    let mut fonts = FontMap::get_default().unwrap().list_families().into_iter().map(|x| x.get_name().unwrap()).collect::<Vec<_>>();
    fonts.sort();
    let w = fonts.len().to_string().len();
    for (i, x) in fonts.iter().enumerate() {
        println!("{index:>width$}: {font}", index=i, width=w, font=x);
    }
}
