class Zetamac < Formula
  desc "Short description of your app"
  homepage "Zetamac timed arithmetic test and practice"
  url "https://github.com/DIvkov575/zetamac-rs/archive/refs/tags/{{version}}.tar.gz"
  sha256 "{{sha256}}"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
    bin.install_symlink bin/"zetamac-rs" => "zetamac"

  end

  test do
    assert_match version.to_s, shell_output("#{bin}/zetamac-rs --version")
  end

end