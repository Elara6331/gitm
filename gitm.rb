#!/usr/bin/ruby

# Gitm. Automatic git mirroring script.
#
# Copyright (C) 2021 Arsen Musayelyan <arsen@arsenm.dev>
#
# This program is free software: you can redistribute it and/or modify it under
# the terms of the GNU General Public License as published by the Free Software
# Foundation, either version 3 of the License, or (at your option) any later
# version.
#
# This program is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
# FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

# You should have received a copy of the GNU General Public License along with
# this program. If not, see <https://www.gnu.org/licenses/>.


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
  repos.each { |name, _| system "git push #{name} #{branch}", *args[1...] }
when "init"
  log "Intercepted init command"
  system "git init", *args[1...]
  repos.each { |name, repo| system "git remote add #{name} #{repo}" }
  system "git fetch origin"
  system "git checkout master"
else
  system "git", *args
end
