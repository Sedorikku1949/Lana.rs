local default_str = "Hello World!"

res = ""

for i = 1, #default_str do
  res = default_str:sub(i,i) .. res
end