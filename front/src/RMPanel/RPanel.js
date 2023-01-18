import React from 'react';
import styled from 'styled-components';

import RMainFuncPanel from "./RMainPanel";
import RBackFuncPanel from "./RBackPanel";
import RHoverFuncPanel from "./RHoverPanel";

const RSideFuncPanel = styled.div`
  position: fixed;
  right: 0;
  width: 30rem;
  top: 50%;
  height: 90%;
  transform: translateY(-50%);
  pointer-events: none;
  //background: #FFFFFFa0;
  //box-shadow: 0 0 1rem 0.2rem #00000040;
`

function RPanel() {
    return (
        <RSideFuncPanel>
            <RHoverFuncPanel/>
            <RBackFuncPanel/>
            <RMainFuncPanel/>
        </RSideFuncPanel>
    )
}

export default RPanel