-- Define the array of parameters
local params = {
    "param1=01",
    "param2=02",
    "param3=03",
    "param4=04"
}

-- Function to get a random parameter from the array
local function get_random_param()
    return params[math.random(1, #params)]
end

-- Function to generate the request
request = function()
    local random_param = get_random_param()
    local url = "/tx?" .. random_param
    return wrk.format("GET", url)
end

-- wrk -t12 -c400 -d30s -s get_random_param.lua http://localhost:8080