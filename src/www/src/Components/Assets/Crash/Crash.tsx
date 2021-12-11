//     ______                    __  
//    / ____/_____ ____ _ _____ / /_ 
//   / /    / ___// __ `// ___// __ \
//  / /___ / /   / /_/ /(__  )/ / / /
//  \____//_/    \__,_//____//_/ /_/ 
//                                   

// Library Imports
import React from 'react';
//import { FiRefreshCcw as Refresh } from 'react-icons/fi';
//import { BiRefresh as Refresh } from 'react-icons/bi';
//import { IoRefresh as Refresh } from 'react-icons/io5';
import { RiRefreshLine as Refresh } from 'react-icons/ri';

// Image
import { ReactComponent as CrashIcon } from './assets/oops.svg';
// CSS
import './crashed.css';

const Crash = () => {
    return(
        <div className="crash-container">
            <CrashIcon />
            <span className="crash-message">
                Failed to get your data.. sorry?!
            </span>
            <button onClick={() => window.location.reload()}>
                <Refresh />
            </button>
        </div>
    )
}

export default Crash
