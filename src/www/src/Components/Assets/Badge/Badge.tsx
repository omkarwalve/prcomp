/** `Badge` - A mini circle that shows count on an icon
 *   To be embedded inside an svg component */
import './badge.css';
const Badge = ({data}: {data: number}) => {
    return( <span className="badge-circle"> {data} </span>)
}

export default Badge;
