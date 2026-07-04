# Homebrew formula for flyloft
# Updated automatically by the release workflow when signing is enabled
# macOS only (arm64). Linux users: download from GitHub releases.

class Flyloft < Formula
  desc "Curated retrieval substrate with hybrid search and a curation surface"
  homepage "https://github.com/arcavenae/flyloft"
  url "https://github.com/arcavenae/flyloft/releases/download/TAG_PLACEHOLDER/flyloft-darwin-arm64"
  version "VERSION_PLACEHOLDER"
  sha256 "SHA256_ARM64_PLACEHOLDER"
  license "MIT"

  def install
    bin.install "flyloft-darwin-arm64" => "flyloft"
  end

  test do
    assert_match "flyloft", shell_output("#{bin}/flyloft --version 2>&1")
  end
end
