import React from 'react';
import styled from 'styled-components';

const RoutesDiv = styled.div`
  position: relative;
  height: 100%;
  width: 100%;
  box-sizing: border-box;
`
const RoutesPanel = styled.div`
  position: absolute;
  height: 60%;
  width: 90%;
  background: #f6f6f660;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  display: flex;
  padding: 0 10%;
  border-radius: 1rem;
  //border-style: solid;
  box-sizing: border-box;
`
const RouteIconButton = styled.button`
  position: relative;
  width: 5rem;
  height: 40%;
  margin: auto;
  border-style: hidden hidden hidden solid;
  //border: transparent;
  //border-width: 2px;
  background: #00000000;
`


function SRoutPanel({routes}) {

    return(
        <RoutesDiv>
            <RoutesPanel>
                {Object.entries(routes).map(
                    ([k, v]) => <RouteIconButton key={k} onClick={()=>v.whenClick()}>
                        {v.text}
                    </RouteIconButton>
                )}
            </RoutesPanel>
        </RoutesDiv>
    )
}

export default SRoutPanel