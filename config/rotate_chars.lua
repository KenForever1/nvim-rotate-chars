-- print(vim.inspect(vim.api.nvim_list_runtime_paths()))
vim.opt.runtimepath:append(',~/.config/nvim/lua')

function processString(input)
    local boolStr, numStr = input:match("^(%S+)%s+(%S+)$")
    
    if not boolStr or not numStr then
        error("Input string does not contain exactly two parts.")
    end
    
    local boolValue
    if boolStr:lower() == "true" then
        boolValue = true
    elseif boolStr:lower() == "false" then
        boolValue = false
    else
        error("Invalid boolean string: " .. boolStr)
    end
    
    local numValue = tonumber(numStr)
    if not numValue then
        error("Invalid number string: " .. numStr)
    end
    
    return boolValue, numValue
end

vim.keymap.set('v', '<Leader>r', function()
  -- 请求用户输入数字参数
  vim.ui.input({ prompt = 'Enter direction and rotation number: ' }, function(input)
    print("bool_val:" .. input)
    local bool_val, number_arg = processString(input)

    print("bool_val:" .. tostring(bool_val))
    --print("bool_val:" .. number_arg)

    if number_arg == nil then
      -- 如果输入不是有效数字，则提示错误并退出
      print("Error: Please enter a valid number.")
      return
    end

    -- 获取选区的起始和结束行
    vim.cmd([[ execute "normal! \<ESC>" ]])
    local mode = vim.fn.visualmode()
    local start_line = vim.fn.getpos("'<")[2] - 1
    local end_line = vim.fn.getpos("'>")[2]

    print("start " .. start_line .. "end " .. end_line)
    -- 调用插件的函数，将输入的数字作为参数
    require("nvim_rotate_chars").RotateCharsWithRange(bool_val, number_arg, start_line, end_line)
  end)
end, { noremap = true, silent = true, desc = "Rotate selected characters" })

vim.api.nvim_create_user_command(
  'RotateChars',
  function(opts)
    -- 检查是否提供了正确数量的参数
    if #opts.fargs < 2 then
      print("Usage: :RotateChars <boolean> <number>")
      return
    end

    -- 将第一个参数解析为布尔值
    local bool_arg = opts.fargs[1] == "true"

    -- 尝试将第二个参数解析为数字
    local num_arg = tonumber(opts.fargs[2])
    if not num_arg then
      print("Error: The second argument must be a number.")
      return
    end

    -- 调用插件的函数
    require("nvim_rotate_chars").RotateChars(bool_arg, num_arg)
  end,
  { range = 2, nargs = "+" }
)
