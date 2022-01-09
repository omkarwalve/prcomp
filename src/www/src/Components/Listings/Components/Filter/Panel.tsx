import useToggle from 'hooks/toggle';
import { Options } from './Menu';
import './panel.css';

const PanelSection = ({filter}: {filter: Options}) => {
  const [cloak, toggleCloak] = useToggle(true);
  const onHeaderClick = () => toggleCloak();
  return (
    <li className="filter-panel-option">
      <h5 className={`filter-panel-option-header ${!cloak ? 'active' : ''}`} onClick={onHeaderClick}>{filter.name}</h5>
      <span className={`filter-panel-option-content ${!cloak ? 'active' : ''}`}>{filter.target}</span>
    </li>
  )
}

/** ### Filters Panel [*Desktop*]
 * Filter Menu options for screens greater than 992px wide.*/
const Panel = ({filters}: {filters: Options[]}) => {
  return (
    <ul className="filter-panel">
      {
        filters.map(filter => {
          return (
            <PanelSection filter={filter} />
          )
        })
      }
    </ul>
  )
}
export default Panel;