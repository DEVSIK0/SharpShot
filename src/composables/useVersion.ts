export function useVersion() {
  const version = import.meta.env.PACKAGE_VERSION || "0.0.0";
  
  return {
    version
  };
}
