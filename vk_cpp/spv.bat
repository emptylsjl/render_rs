

set glslc="%VK_SDK_PATH%/Bin/glslc.exe"
set cdir="%cd%/sample/uniform/"

%glslc% %cdir%shader.vert -o %cdir%vert.spv
%glslc% %cdir%shader.frag -o %cdir%frag.spv
