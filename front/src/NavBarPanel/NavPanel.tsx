import React from 'react';
import styled from 'styled-components';

const NMDiv = styled.div`
  position: fixed;
  top: 50%;
  left: 0.5%;
  transform: translateY(-50%);
  min-height: 5rem;
  width: 4rem;
  border-radius: 1rem;
  background: #ffffff40;
`
const NMPanel = styled.div`
  width: 100%;
  height: 100%;
  padding: 1rem 0;
  mask-image: linear-gradient(to bottom, transparent 5%, black 10%, black 90%, transparent 95%);
`
const RouteButton = styled.button`
  width: 70%;
  border: transparent;
  border-radius: 25%;
  aspect-ratio: 1/1;
  background: #dca723;
  margin: 5% 15%;
`

function abc() {

    fetch("127.0.0.1:5000/static/a.txt")
        .then((response) => {
            if (response.status === 200) {
                // return response.text();
                console.log(response.status)
            } else {
                throw new Error('Someth API server!');
            }
        })
        .then((t) => {
            console.log(t)
        }).catch((error) => {
        console.error(error);
    });
}

function NavMainPanel() {

    const list = {aa: 'aa', bb: 'bb'}

    return(
        <NMDiv><NMPanel>
            {Object.entries(list).map(([k, v]) =>
                <RouteButton key={k} onClick={()=>abc()}>
                    {v}
                </RouteButton>
            )}
        </NMPanel></NMDiv>
    )
}

export default NavMainPanel