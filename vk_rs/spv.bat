

set glslc="C:/Program Files/VulkanSDK/1.3.236.0/Bin/glslc.exe"
set cdir="%cd%/src/shader/"

%glslc% %cdir%vert_test.vert -o %cdir%vert.spv
%glslc% %cdir%frag_test.frag -o %cdir%frag.spv