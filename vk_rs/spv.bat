

set glslc="%VK_SDK_PATH%/Bin/glslc.exe"
set cdir="%cd%/src/shader/"

%glslc% %cdir%vert_test.vert -o %cdir%vert.spv
%glslc% %cdir%frag_test.frag -o %cdir%frag.spv