class ObsCutter < Formula
  desc "Split 32:9 OBS recordings into two separate 16:9 videos"
  homepage "https://github.com/yourusername/obs-cutter"
  # For local development, we'll build from the current directory
  # This formula should be in the same directory as Cargo.toml
  version "1.0.0"
  license "MIT"

  depends_on "rust" => :build
  depends_on "ffmpeg"

  def install
    # Build and install from the current directory
    system "cargo", "install", "--locked", "--root", prefix, "--path", buildpath.parent.parent.parent
  end

  test do
    assert_match "obs-cutter 1.0.0", shell_output("#{bin}/obs-cutter --version")
  end
end
