import React from 'react';
import './Loading.css';

const Loading = () => {
        return (
          <div className='spinnerContainer'>
            <div className='loader'></div>
            <p className='loaderText'>Fetching..</p>
          </div>
        )
}

export default Loading;
