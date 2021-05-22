#!/usr/bin/ruby

require 'toml'
require 'colorize'

args = ARGV
cfgName = ".gitm.toml"

def log(str)
  txt = "[gitm]".cyan
  puts "#{txt} #{str}"
end

File.open(cfgName, "a") {} unless File.exists? cfgName

cfgData = TOML::Parser.new(File.read(cfgName)).parsed
repos = cfgData["repos"] || {}
branch = cfgData["defaultBranch"] || "master"
if repos["origin"].nil?
  log "Error: origin repo required"
  exit 1
end

if repos.length < 1
  puts "Please add repos to the #{cfgName} file"
  exit 1
end

case args[0]
when "push"
  log "Intercepted push command"
  repos.each { |name, _| system "git push #{name} #{branch}" }
when "init"
  log "Intercepted init command"
  system "git init"
  repos.each { |name, repo| system "git remote add #{name} #{repo}" }
  system "git fetch origin"
  system "git checkout master"
else
  system "git", *args
end