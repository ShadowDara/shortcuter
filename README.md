# shortcuter

a little Rust programm to easily make shortcuts ore easy startable scripts

### Config
```lua
-- Table for the Export Values
c = {}
c.wait_time = 5
c.show_info = true
c.loglevel = 3

-- Lua function 1
function greet()
    print("Hello from Lua!")
    for i = 1, 3 do
        print("Loop", i)
    end
end

-- Lua Function 2
function add_numbers(a, b)
    local sum = a + b
    print("Sum:", sum)
    return sum
end

-- Add functions to the callable lua function table
c.funcs = {
    greet = greet,
    add = add_numbers
}
```
