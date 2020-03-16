require 'spec_helper'
require 'liquid/rust'

RSpec.describe Liquid::Rust do
  it "returns template source" do
    t = Liquid::Rust::Template.parse("source")
    expect(t.source).to eq("source")
  end

  it "renders stuff" do
    t = Liquid::Rust::Template.parse("Liquid! {{num | minus: 2}}")
    expect(t.render({ "num" => 4 })).to eq("Liquid! 2")
  end
end
