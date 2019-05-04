def replace(filename, section_name, content_new)
  filename = File.join(File.dirname(__FILE__), "..", filename)

  file_old = File.read filename
  line_start = "// BEGIN CODEGEN(#{section_name})"
  line_end = "// END CODEGEN(#{section_name})"
  lines = file_old.split("\n")

  i_start = lines.index(line_start)
  i_end = lines.index(line_end)

  if i_start.nil? or i_end.nil?
    abort "Didn't find the codegen tags for #{section_name}."
  end

  # remove the lines
  lines.slice!(i_start..i_end)

  # insert the new lines
  content_lines = content_new.split("\n")
  content_lines = [line_start, *content_lines, line_end]
  lines.insert(i_start, *content_lines)

  # write the new content
  file_new = lines.join("\n")
  File.open(filename, "w") do |file|
    file.write file_new
  end
end
