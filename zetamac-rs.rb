class YourApp < Formula
  desc "Short description of your app"
  homepage "Zetamac timed arithemtic test and pratice"
  url "https://github.com/divkov575/zetamac-rs/releases/download/v0.1.0/zetamac-rs.tar.gz"
  sha256 "checksum_of_the_tar_gz_file"
  license "MIT"

  license "MIT"

  depends_on "rust" => :build
  depends_on "pkg-config" => :build   # If your crate depends on native libraries
  depends_on "openssl@3"               # Example of a common dependency; adjust as needed

  def install
    # Build and install your Rust crate using cargo
    system "cargo", "install", *std_cargo_args(path: ".")
  end

  test do
    # Check the version output to verify installation worked
    assert_match version.to_s, shell_output("#{bin}/your_app --version")

    # Optionally test a basic command or behavior of your app
    output = shell_output("#{bin}/your_app some_command")
    assert_match "expected output", output
  end
end