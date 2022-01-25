//   .---.  .----.  .----. .-.  .---. .-..----. 
//  {_   _}/  {}  \/  {}  \| | {_   _}| || {}  }
//    | |  \      /\      /| `--.| |  | || .--' 
//    `-'   `----'  `----' `----'`-'  `-'`-'    

// Library Imports
import React,{memo} from 'react';

// File Imports
import './tooltip.css';

const Tooltip = ({text}) => {
    return(
        <span className="tooltip">{text}</span>
    )
}

export default memo(Tooltip);
