

set glslc="%VK_SDK_PATH%/Bin/glslc.exe"
set cdir="%cd%/src/shader/"

%glslc% %cdir%vert.vert -o %cdir%vert.spv
%glslc% %cdir%frag.frag -o %cdir%frag.spv