require 'spec_helper'
require 'liquid/rust'

RSpec.describe Liquid::Rust do
  it "returns template source" do
    t = Liquid::Rust::Template.parse("source")
    expect(t.source).to eq("source")
  end
end
