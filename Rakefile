# frozen_string_literal: true
require "rake/extensiontask"


SPEC = Gem::Specification.load("magnus_kwargs_bug.gemspec")


Rake::ExtensionTask.new("magnus_kwargs_bug", SPEC) do |ext|
  ext.source_pattern = "*.{rs,toml}"

  ext.lib_dir = File.join("lib", "magnus_kwargs_bug")

  ext.cross_compile = true
  ext.cross_platform = %w[arm64-darwin]

  ext.config_script = ENV["ALTERNATE_CONFIG_SCRIPT"] || "extconf.rb"

  # remove things not needed for precompiled gems
  ext.cross_compiling do |spec|
    spec.files.reject! { |file| File.fnmatch?("*.tar.gz", file) }
    spec.dependencies.reject! { |dep| dep.name == "rb-sys" }
  end
end


require "bundler/gem_tasks"
require "rake/testtask"

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/test_*.rb"]
end

require "rubocop/rake_task"

RuboCop::RakeTask.new

task default: %i[test rubocop]
