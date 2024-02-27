import { View, Text } from "react-native";
import { styled } from "nativewind";
import { Link } from "expo-router";

const StyledText = styled(Text);
const StyledView = styled(View);
const StyledLink = styled(Link);
const HomePage = () => {
  return (
    <StyledView className="flex-1 items-center">
      <StyledText className="text-2xl font-bold my-32">Home Page</StyledText>
      <StyledLink href="/camera" className="absolute bottom-0 mb-10">
        Camera
      </StyledLink>
    </StyledView>
  );
};

export default HomePage;
