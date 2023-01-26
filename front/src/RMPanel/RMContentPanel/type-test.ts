import React from 'react';
import styled from 'styled-components';
// import bezier from "bezier-easing";

const Contain = styled.div`
  position: absolute;
  width: 100%;
  height: 100%;
`

const RectBlock = styled.div`
  position: absolute;
  top: 50%;
  width: 50px;
  height: 50px;
  background: #1f1e33;
  transform: translateY(-50%);
`


function TestPanel() {

    return (
        <Contain>
            <RectBlock/>
        </Contain>
    )
}

export default TestPanel