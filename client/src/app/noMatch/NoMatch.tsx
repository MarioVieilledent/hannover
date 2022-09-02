import React from 'react';
import './NoMatch.scss';

class NoMatch extends React.Component {
  render() {
    return (
      <div>
        <span>Page inexistante.</span>
        <a href="/">Revenir à la page d&apos;accueil</a>
      </div>
    )
  }
}
export default NoMatch;