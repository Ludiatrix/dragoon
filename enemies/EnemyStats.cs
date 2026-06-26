using Godot;

namespace ExampleProject
{
	[GlobalClass]
	public partial class EnemyStats : Resource
	{
		[Export]
		public int MaxHealth { get; set; }

		[Export]
		public Resource Model { get; set; }

		public EnemyStats() : this(0, null) {}

		public EnemyStats(int maxHealth, Resource model)
		{
			MaxHealth = maxHealth;
			Model = model;
		}
	}
}
