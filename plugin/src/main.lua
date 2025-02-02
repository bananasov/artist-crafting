local logger = require("artist.lib.log")
local keybinding = require("metis.input.keybinding")

local log = logger.get_logger("artist-crafting")

-- Love ya wojbie
local function findupvalue(fun, target)
    for i = 1, 100 do
        local name, value = debug.getupvalue(fun, i)
        if name == target then return value end
        if name == nil then
            log("failed to find %s", target)
            error()
        end
    end
end

return function(context)
    log("Starting up crafting plugin")


    local queue = findupvalue(context._main_pool.spawn, "queue")
    local target = queue[#queue]

    local ui = findupvalue(target, "ui")
    local push_furnace = findupvalue(target, "push_furnace")
    local gui = findupvalue(target, "gui")
    local width = findupvalue(target, "width")
    local item_list = findupvalue(target, "item_list")

    local function example()
        log("Ctrl+Shift+R was run")
    end

    context:spawn(function()
        ui:push {
            keymap = keybinding.create_keymap {
                ["C-d"] = function()
                    ui:pop()
                    ui:pop()
                end,
                ["C-S-f"] = push_furnace,
                ["C-S-r"] = example,
            },
            children = {
                gui.Input {
                    x = 1, y = 1, width = width, fg = "black", bg = "white", placeholder = "Search...",
                    changed = function(value) item_list:set_filter(value) end,
                },
                item_list,
            },
        }
    end)

    log("Injected modified main frame")
end
