class YourApp < Formula
  desc "Short description of your app"
  homepage "Zetamac timed arithemtic test and pratice"
  url "https://github.com/yourusername/your_app/releases/download/v0.1.0/your_app.tar.gz"
  sha256 "checksum_of_the_tar_gz_file"
  license "MIT"

  def install
    bin.install "your_app"
  end

  test do
    system "#{bin}/your_app", "--version"
  end
end