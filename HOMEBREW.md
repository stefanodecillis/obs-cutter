# Homebrew Installation Guide

This guide shows you how to make obs-cutter installable via Homebrew.

## Option 1: Local Installation (Quick Test)

For testing locally without pushing to GitHub:

```bash
# Install directly from the local formula
brew install --formula Formula/obs-cutter.rb
```

To uninstall:
```bash
brew uninstall obs-cutter
```

## Option 2: Create Your Own Tap (Recommended for Distribution)

A Homebrew "tap" is a GitHub repository that contains formulae. This is the best way to distribute your tool.

### Step 1: Push Your Code to GitHub

1. Create a new GitHub repository named `obs-cutter`
2. Push your code:

```bash
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/YOURUSERNAME/obs-cutter.git
git push -u origin main
```

### Step 2: Create a GitHub Release

1. Go to your repository on GitHub
2. Click "Releases" → "Create a new release"
3. Create a tag: `v1.0.0`
4. Publish the release

### Step 3: Get the Tarball SHA256

```bash
# Download the release tarball and get its hash
curl -L https://github.com/YOURUSERNAME/obs-cutter/archive/refs/tags/v1.0.0.tar.gz | shasum -a 256
```

Copy this SHA256 hash.

### Step 4: Create a Homebrew Tap Repository

1. Create a new GitHub repository named `homebrew-tools` (or `homebrew-tap`)
   - The name MUST start with `homebrew-`

2. Clone it locally:

```bash
git clone https://github.com/YOURUSERNAME/homebrew-tools.git
cd homebrew-tools
```

3. Create the formula file `obs-cutter.rb`:

```ruby
class ObsCutter < Formula
  desc "Split 32:9 OBS recordings into two separate 16:9 videos"
  homepage "https://github.com/YOURUSERNAME/obs-cutter"
  url "https://github.com/YOURUSERNAME/obs-cutter/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "PASTE_YOUR_SHA256_HERE"
  license "MIT"
  head "https://github.com/YOURUSERNAME/obs-cutter.git", branch: "main"

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

4. Commit and push:

```bash
git add obs-cutter.rb
git commit -m "Add obs-cutter formula"
git push
```

### Step 5: Install via Homebrew

Now anyone can install your tool:

```bash
# Add your tap
brew tap YOURUSERNAME/tools

# Install obs-cutter
brew install obs-cutter
```

Or in one command:
```bash
brew install YOURUSERNAME/tools/obs-cutter
```

### Step 6: Update Your obs-cutter README

Add this to your obs-cutter README.md installation section:

```markdown
### Install via Homebrew

```bash
brew tap YOURUSERNAME/tools
brew install obs-cutter
```
```

## Option 3: Install from HEAD (Development Version)

Users can install the latest development version:

```bash
brew install --HEAD YOURUSERNAME/tools/obs-cutter
```

## Updating Your Formula for New Releases

When you release a new version:

1. Create a new GitHub release (e.g., `v1.1.0`)
2. Get the new tarball SHA256:
   ```bash
   curl -L https://github.com/YOURUSERNAME/obs-cutter/archive/refs/tags/v1.1.0.tar.gz | shasum -a 256
   ```
3. Update the formula in your tap repository:
   - Update the `url` version number
   - Update the `sha256` hash
   - Update the `version` if specified
4. Commit and push the changes

Users can then update:
```bash
brew update
brew upgrade obs-cutter
```

## Testing Your Formula

Before publishing, test your formula:

```bash
# Audit the formula for issues
brew audit --strict obs-cutter

# Install and test
brew install obs-cutter
brew test obs-cutter

# Uninstall
brew uninstall obs-cutter
```

## Example Complete Workflow

Here's a complete example with username "stefano":

1. **Create repos:**
   - `github.com/stefano/obs-cutter` (your tool)
   - `github.com/stefano/homebrew-tools` (your tap)

2. **Create release v1.0.0** in obs-cutter repo

3. **Get SHA256:**
   ```bash
   curl -L https://github.com/stefano/obs-cutter/archive/refs/tags/v1.0.0.tar.gz | shasum -a 256
   ```

4. **Create formula** in homebrew-tools repo

5. **Users install:**
   ```bash
   brew tap stefano/tools
   brew install obs-cutter
   ```

## Quick Reference

```bash
# Install from local formula (testing)
brew install --formula Formula/obs-cutter.rb

# Install from tap
brew tap YOURUSERNAME/tools
brew install obs-cutter

# Install in one command
brew install YOURUSERNAME/tools/obs-cutter

# Install development version
brew install --HEAD YOURUSERNAME/tools/obs-cutter

# Update
brew update && brew upgrade obs-cutter

# Uninstall
brew uninstall obs-cutter
brew untap YOURUSERNAME/tools  # Optional: remove tap
```

## Advantages of Homebrew Distribution

1. **Easy installation** - Single command for users
2. **Automatic updates** - Users get updates via `brew upgrade`
3. **Dependency management** - Homebrew handles FFmpeg installation
4. **Version management** - Users can install specific versions
5. **Trustworthy** - Familiar installation method for macOS users
6. **No manual PATH** - Automatically installed to PATH

## Alternative: Submit to Homebrew Core

For widely-used tools, you can submit to the official Homebrew repository:

1. Fork `homebrew/homebrew-core`
2. Add your formula to `Formula/`
3. Submit a pull request

Requirements:
- Active maintenance
- Significant user base
- Notable application
- Good documentation

For personal/niche tools, a custom tap is recommended.
