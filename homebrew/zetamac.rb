class Zetamac < Formula
  desc "Short description of your app"
  homepage "Zetamac timed arithmetic test and practice"
  url "https://github.com/DIvkov575/zetamac-rs/archive/refs/tags/0.5.1.tar.gz"
  sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
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