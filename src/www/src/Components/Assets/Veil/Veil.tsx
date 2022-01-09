
import "./veil.css";
const Veil = ({cloak,toggler}:{cloak: boolean,toggler: () => void}) => {
    const onVeilClick = () => { toggler(); };
    return (
        <div onClick={onVeilClick} className={`veil ${cloak ? 'cloak' : ''}`}></div>
    )
}

export default Veil;
