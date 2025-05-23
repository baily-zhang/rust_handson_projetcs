// ✅ 第一步：定义类型
type Post = {
  title: string;
  date: string;
  summary: string;
};

export default async function Home() {
  let posts: Post[] = [];

  try {
    const res = await fetch("http://localhost:3001/api/posts", {
      cache: "no-store",
    });

    if (!res.ok) throw new Error("Failed to fetch");
    posts = await res.json();
  } catch (err) {
    console.error("❌ Failed to load posts:", err);
  }

  return (
    <main className="max-w-2xl mx-auto p-6">
      <h1 className="text-3xl font-bold mb-6 text-gray-800">My Blog</h1>
      <ul className="space-y-4">
        {posts.map((post: Post, idx: number) => (
          <li key={idx} className="border-b pb-4">
            <h2 className="text-xl font-semibold text-blue-600">{post.title}</h2>
            <p className="text-sm text-gray-500">{post.date}</p>
            <p className="mt-1 text-gray-700">{post.summary}</p>
          </li>
        ))}
      </ul>
    </main>
  );
}
