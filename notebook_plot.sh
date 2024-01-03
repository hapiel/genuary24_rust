# open cmd, enter this script name, followed by svg file
# for plotting in the sketchbook
# unlike the original blender script, this one doesn't scale
vpype read $1 rotate 180 linemerge linesimplify reloop linesort write "%prop.vp_source.with_stem(prop.vp_source.stem + '_vpype')%" gwrite $1.gcode