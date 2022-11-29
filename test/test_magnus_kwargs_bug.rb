# frozen_string_literal: true

require "test_helper"

class TestMagnusKwargsBug < Minitest::Test
  def test_that_it_can_accept_kwargs
    MagnusKwargsBug::Selector.new(match_text_within: "foo")
  end
end
