require "liquid/rust/version"
require 'rutie'

module Liquid
  module Rust
    Rutie.new(:liquid_rust).init 'Init_liquid_rust', "#{__dir__}/.."
  end
end
