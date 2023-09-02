# Characters

<input
    type="search"
    placeholder="Type in the character name here..."
    style="width: 50%"
    v-model="search"
/>

<div id="character-list">
  <div 
      class="character-items"
      v-for="c in characters"
      @click="clickCharacter(c.name)"
      v-show="!search.length || c.name.toLowerCase().includes(search)" 
  >
    <div :class="`kingdom ${c.kingdom} ${c.monarch && 'zhu'}`"></div>
    <div>{{ c.name }}</div>
  </div>
</div>

<style>
  #character-list {
    width: 75%;
    border: 1px solid black;
    border-radius: 10px;
  }

  .character-items {
    display: flex;
    align-items: center;
    cursor: pointer;
    padding: 15px;
    border-bottom: 1px solid black
  }

  .character-items:last-child {
    border-bottom: none
  }

  .character-items:hover {
    background-color: black;
  }

  .kingdom {
    width: 10px;
    height: 10px;
    margin-right: 10px;
  }

  .shu {
    background-color: red;
  }

  .wei {
    background-color: blue;
  }

  .wu {
    background-color: green;
  }
  
  .kingdomless {
    background-color: gray;
  }

  .zhu {
    border: 1px solid yellow;
  }

  input {
    font-size:18px;
    padding:10px 10px 10px 5px;
    display:block;
    width:300px;
    border:none;
    border-bottom:1px solid #757575;
  }
  input:focus 		{ outline:none; }
</style>