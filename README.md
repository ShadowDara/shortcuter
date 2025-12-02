# shortcuter

a little Rust programm to easily make shortcuts ore easy startable scripts

- Run with `config` for Infos
- Run with `list` to List all registered Functions

### Setup
run `sc config` and create a new config File. Paste the default downbelow.
Now have fun!

### Config
```lua
-- Table for the Export Values
c = {}

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

### Contributing

Feel free to create Issues or contribute !
