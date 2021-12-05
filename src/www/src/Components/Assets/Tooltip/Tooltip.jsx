//   .---.  .----.  .----. .-.  .---. .-..----. 
//  {_   _}/  {}  \/  {}  \| | {_   _}| || {}  }
//    | |  \      /\      /| `--.| |  | || .--' 
//    `-'   `----'  `----' `----'`-'  `-'`-'    

// Library Imports
import React from 'react';

// File Imports
import './tooltip.css';


const Tooltip = ({text}) => {
    return(
        <span className="tooltip">{text}</span>
    )
}

export default Tooltip
