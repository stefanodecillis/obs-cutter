# Homebrew Quick Start

## TL;DR - Make Your Tool Installable via Homebrew

Follow these steps to allow others to install your tool with `brew install`:

### 1. Push Your Code to GitHub

```bash
# Initialize git (if not already done)
git init
git add .
git commit -m "Initial commit"

# Create a GitHub repository and push
git remote add origin https://github.com/YOURUSERNAME/obs-cutter.git
git branch -M main
git push -u origin main
```

### 2. Create a GitHub Release

1. Go to `https://github.com/YOURUSERNAME/obs-cutter/releases/new`
2. Tag: `v1.0.0`
3. Title: `v1.0.0`
4. Click "Publish release"

### 3. Get the Release Tarball SHA256

```bash
curl -L https://github.com/YOURUSERNAME/obs-cutter/archive/refs/tags/v1.0.0.tar.gz | shasum -a 256
```

Copy the hash that's printed.

### 4. Create Your Homebrew Tap Repository

```bash
# Create a new GitHub repository named: homebrew-tools
# Then clone it:
git clone https://github.com/YOURUSERNAME/homebrew-tools.git
cd homebrew-tools
```

### 5. Create the Formula File

Create `obs-cutter.rb` with this content (replace YOURUSERNAME and SHA256):

```ruby
class ObsCutter < Formula
  desc "Split 32:9 OBS recordings into two separate 16:9 videos"
  homepage "https://github.com/YOURUSERNAME/obs-cutter"
  url "https://github.com/YOURUSERNAME/obs-cutter/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "PASTE_YOUR_SHA256_HERE"
  license "MIT"

  depends_on "rust" => :build
  depends_on "ffmpeg"

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match "obs-cutter 1.0.0", shell_output("#{bin}/obs-cutter --version")
  end
end
```

### 6. Push the Formula

```bash
git add obs-cutter.rb
git commit -m "Add obs-cutter formula"
git push
```

### 7. Done! Users Can Now Install

```bash
brew tap YOURUSERNAME/tools
brew install obs-cutter
```

Or in one line:
```bash
brew install YOURUSERNAME/tools/obs-cutter
```

## Example with Real Username

If your GitHub username is `stefano`:

```bash
# Users install with:
brew tap stefano/tools
brew install obs-cutter

# Or:
brew install stefano/tools/obs-cutter
```

## Updating for New Releases

When you release v1.1.0:

1. Create GitHub release with tag `v1.1.0`
2. Get new SHA256:
   ```bash
   curl -L https://github.com/YOURUSERNAME/obs-cutter/archive/refs/tags/v1.1.0.tar.gz | shasum -a 256
   ```
3. Update `obs-cutter.rb` in your tap:
   - Change URL version to `v1.1.0`
   - Update SHA256
4. Commit and push

Users update with:
```bash
brew update
brew upgrade obs-cutter
```

## See Also

- Full guide: [HOMEBREW.md](HOMEBREW.md)
- Formula files are in: `Formula/` directory
  - `obs-cutter-github.rb` - Template for GitHub-based tap
