/**
 * # Adapterパターン
 * 既存のクラス(Banner)を別の用途で利用したいときに、そのクラスをラップするクラス(PrintBanner)を作って挙動を変更するパターン。
 * ラッパークラスはインターフェースを介して利用する
 */
pub fn main() {
	let banner = Banner::new("hello");
	let print_banner: Box<dyn Print> = Box::new(PrintBanner::new(banner));
	println!("print_weak: {}", print_banner.print_weak());
	println!("print_strong: {}", print_banner.print_strong());
}

/**
 * Bannerクラスの挙動を変更するためのラッパークラス
 */
struct PrintBanner {
	banner: Banner,
}
impl PrintBanner {
	fn new(banner: Banner) -> Self {
		return PrintBanner { banner };
	}
}

/**
 * PrintBannerの挙動を定義するトレイト
 */
trait Print {
	fn print_weak(&self) -> String;
	fn print_strong(&self) -> String;
}

impl Print for PrintBanner {
	fn print_weak(&self) -> String {
		return self.banner.show_with_paren();
	}

	fn print_strong(&self) -> String {
		return self.banner.show_with_aster();
	}
}

/**
 * 既存のクラス
 */
struct Banner {
	message: String,
}

impl Banner {
	fn new(message: &str) -> Self {
		return Banner {
			message: format!("{}", message),
		}
	}

	fn show_with_paren(&self) -> String {
		return format!("({})", self.message);
	}

	fn show_with_aster(&self) -> String {
		return format!("*{}*", self.message);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_print_weak() {
		let banner = Banner::new("hello");
		let print_banner: Box<dyn Print> = Box::new(PrintBanner::new(banner));
		assert_eq!(print_banner.print_weak(), "(hello)")
	}

	#[test]
	fn test_print_strong() {
		let banner = Banner::new("hello");
		let print_banner: Box<dyn Print> = Box::new(PrintBanner::new(banner));
		assert_eq!(print_banner.print_strong(), "*hello*");
	}
}