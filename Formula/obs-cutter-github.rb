# This is the formula for when the project is hosted on GitHub
# Replace "yourusername" with your actual GitHub username
# Replace the sha256 with the actual tarball hash after creating a release

class ObsCutter < Formula
  desc "Split 32:9 OBS recordings into two separate 16:9 videos"
  homepage "https://github.com/yourusername/obs-cutter"
  url "https://github.com/yourusername/obs-cutter/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"
  license "MIT"
  head "https://github.com/yourusername/obs-cutter.git", branch: "main"

  depends_on "rust" => :build
  depends_on "ffmpeg"

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match "obs-cutter 1.0.0", shell_output("#{bin}/obs-cutter --version")
  end
end
