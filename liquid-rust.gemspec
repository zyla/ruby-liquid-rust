require_relative 'lib/liquid/rust/version'

Gem::Specification.new do |spec|
  spec.name          = "liquid-rust"
  spec.version       = Liquid::Rust::VERSION
  spec.authors       = ["Maciej Bielecki"]
  spec.email         = ["zyla@prati.pl"]

  spec.summary       = %q{Ruby binding to the Rust implementation of Liquid templating language}
  spec.description   = %q{
    Ruby binding to the Rust implementation of Liquid templating language.
    For performance reasons.
  }
  spec.homepage      = "https://github.com/zyla/ruby-liquid-rust"
  spec.required_ruby_version = Gem::Requirement.new(">= 2.3.0")

  spec.metadata["allowed_push_host"] = "TODO: Set to 'http://mygemserver.com'"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/zyla/ruby-liquid-rust.git"
  spec.metadata["changelog_uri"] = "https://github.com/zyla/ruby-liquid-rust"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files         = Dir.chdir(File.expand_path('..', __FILE__)) do
    `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  end
  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
end
