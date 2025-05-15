def main():
    print()
    print("enter mass in kg (rounded to nearest kg): ")
    mass = int(input())
    print()
    print(f"energy = {energy(mass)} joules");


def energy(mass: int) -> int:
    SPEED_OF_LIGHT_MS = 299792458
    return mass * SPEED_OF_LIGHT_MS


if __name__ == "__main__":
    main()
