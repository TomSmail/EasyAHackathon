import styles from "./SearchBar.styles";

export const SearchBar = () => {
  const handleClick = () => {};

  return (
    <styles.SearchBarWrapper>
      <styles.SearchBarContainer>
        <styles.SearchInput type="text" placeholder="Search..." />
      </styles.SearchBarContainer>
    </styles.SearchBarWrapper>
  );
};
