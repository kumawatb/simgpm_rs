using DataFrames, CSV, Plots


data = CSV.read("../output/population.csv",DataFrame)

x_range = maximum(data.x)+1
y_range = maximum(data.y)+1
time_range = maximum(data.time)+1

arr = zeros(Int64,time_range+1,x_range,y_range)

map(eachrow(data)) do row
    arr[row.time+1,row.x+1,row.y+1] += row.pop
end

anim = @animate for i in 1:10000
    heatmap(1:x_range,1:y_range,arr[i,:,:])
end

gif(anim,"anim.gif",fps=60)