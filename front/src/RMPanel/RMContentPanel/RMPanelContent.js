import React from 'react';
import styled from 'styled-components';


const RMIButton = styled.button`
  position: absolute;
  //top: ${props => props.iconStyle?.top || -50}px;
  left: 15%;
  //transform: translateY(${props => props.iconStyle?.transY || 0}px);
  width: 70%;
  aspect-ratio: 1/1;
  background-color: #777777;
  border-radius: 50%;
  border: transparent;
  transition: transform 0.5s;
`
const RMFDiv = styled.div`
  position: relative;
  top: 10%;
  height: ${props => props.compStyle?.Divheight || '20rem'};
  width: 90%;
`
const MFPanel = styled.div`
  position: absolute;
  width: 100%;
  height: 90%;
  border-radius: 5%;
  text-align: center;
  background: #777777;
`

function RMIcon({compStyle, compRef, compKey}) {

    function mainIconOnClick() {
        compRef.panelRef.current.scrollIntoView({behavior: "smooth", block: "center"});
    }

    const iconStyle = {
        top: (compStyle.iconTop[compKey] || '-40')+'px',
        transform: `translateY(${(compStyle.iconY[compKey] || 0)}px)`
    }

    return (
        <RMIButton style={iconStyle} onClick={()=>mainIconOnClick()}>
        </RMIButton>
    )
}

function RMFPanel({compStyle, setState, compRef: {panelRef}}) {


    // useEffect(() => {
    //     setState.sIconTop(panelRef.current.offsetTop)
    //
    //     // eslint-disable-next-line react-hooks/exhaustive-deps
    // }, [])

    return (
        <RMFDiv ref={panelRef}>
            <MFPanel compStyle={compStyle}>
            </MFPanel>
        </RMFDiv>
    )
}

export {RMIcon, RMFPanel}